from fastapi import FastAPI, HTTPException, Depends
import httpx
from pydantic import BaseModel, Field
from sqlalchemy.orm import Session

from . import models, database
from .database import engine

models.Base.metadata.create_all(bind=engine)

app = FastAPI(title="Order Service")

def get_db():
    db = database.SessionLocal()
    try:
        yield db
    finally:
        db.close()

class orderRequest(BaseModel):
    user_id : int
    amount : float = Field(gt=0, description="The amount must be greater than zero")
    
@app.post("/orders")
async def create_order(order: orderRequest, db: Session = Depends(get_db)):
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
                new_order = models.Order(
                    user_id = order.user_id,
                    amount = order.amount,
                    status = "success"
                )

                db.add(new_order)
                db.commit()
                db.refresh(new_order)

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

@app.get("/orders")
def get_orders(db: Session= Depends(get_db)):
    orders = db.query(models.Order).all()
    return orders

