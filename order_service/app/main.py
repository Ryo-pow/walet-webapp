from fastapi import FastAPI, HTTPException
import httpx
from pydantic import BaseModel, Field

app = FastAPI(title="Order Service")

class orderRequest(BaseModel):
    user_id : int
    amount : float = Field(gt=0, description="The amount must be greater than zero")
    
@app.post("/orders")
async def create_order(order: orderRequest):
    rails_base_url = "http://127.0.0.1:3000"
    withdraw_url = f"{rails_base_url}/api/v1/wallets/{order.user_id}/withdraw"

    async with httpx.AsyncClient() as client:
        try:
            response = await client.post(
                withdraw_url,
                json={"amount": order.amount},
                timeout=5.0
            )

            if response.status_code == 200:
                result = response.json()
                return {
                    "message": "Order created and payment successful",
                    "remaining_balance": result.get("balance")
                }
            
            elif response.status_code == 400:
                error_msg = response.json().get("message", "Payment failed")
                raise HTTPException(status_code=400, detail=error_msg)
            
            else:
                raise HTTPException(status_code=500, detail="Wallet service intermal error")
            
        except httpx.ConnectError:
            raise HTTPException(
                status_code=503,
                detail="Wallet service is currently unavailable. Please try again later."
            )

@app.get("/")
def read_root():
    return {"status": "Order Server is running"}
