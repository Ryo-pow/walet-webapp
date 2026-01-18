from fastapi import FastAPI, HTTPException, Depends
import httpx
from pydantic import BaseModel, Field
from sqlalchemy.orm import Session
import redis
import json

from . import models, database
from .database import engine

models.Base.metadata.create_all(bind=engine)

app = FastAPI(title="Order Service")

async def get_latest_price():
    url = "http://localhost:8080/price"
    async with httpx.AsyncClient() as client:
        response = await client.get(url)
        data = response.json()
        return data["price"]

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
    market_price = await get_latest_price()
    total_cost = order.amount * market_price

    redis_client = redis.Redis(host='localhost', port=6379, db=0)
    rails_base_url = "http://127.0.0.1:3000"
    withdraw_url = f"{rails_base_url}/api/v1/wallets/{order.user_id}/withdraw"
    async with httpx.AsyncClient() as client:
        try:
            response = await client.post(
                withdraw_url,
                json={"amount": total_cost},
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

                order_payload = {
                    "id": new_order.id,
                    "user_id": new_order.user_id,
                    "price": market_price,
                    "amount": order.amount,
                    "side": "buy"
                }

                redis_client.lpush("order_queue", json.dumps(order_payload))
                print(f"Order{new_order.id} をRedisキューに投入しました")
                result = response.json()
                return {
                    "message": "Order created and payment successful, and sent to Matching Engine",
                    "order_id": new_order.id,
                    "market_price": market_price,
                    "remaining_balance": result.get("balance")
                }
            
            elif response.status_code == 400:
                error_msg = response.json().get("message", "Payment failed")
                failed_order = models.Order(
                    user_id = order.user_id,
                    amount = order.amount,
                    status = "failed"
                )
                db.add(failed_order)
                db.commit()
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
def get_orders(user_id: int | None,db: Session= Depends(get_db)):
    query = db.query(models.Order)
    if user_id:
        query = query.filter(models.Order.user_id == user_id)
    return query.all()

@app.get("/orders/{order_id}")
def get_order(order_id: int, db: Session = Depends(get_db)):
    order = db.query(models.Order).filter(models.Order.id == order_id).first()
    if order is None:
        raise HTTPException(status_code=404, detail="注文が見つかりません")
    return order
