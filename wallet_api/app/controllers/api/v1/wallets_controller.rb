module Api
  module V1
    class WalletsController < ApplicationController
      # POST /api/v1/wallets/:id/withdraw
      def withdraw
        # params[:id] は FastAPI から送られてくる user_id
        wallet = Wallet.find_by(user_id: params[:id])

        if wallet.nil?
          return render json: { message: "Wallet not found for user_id: #{params[:id]}" }, status: :not_found
        end

        amount = params[:amount].to_f

        # データベースのロックを取り、同時決済による不整合を防ぐ
        wallet.with_lock do
          if wallet.balance >= amount
            wallet.balance -= amount
            wallet.save!
            render json: { status: "success", balance: wallet.balance }
          else
            render json: { message: "Insufficient balance" }, status: :bad_request
          end
        end
      rescue => e
        render json: { message: "Internal Server Error: #{e.message}" }, status: :internal_server_error
      end
    end
  end
end