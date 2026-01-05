# wallet_api/config/routes.rb

Rails.application.routes.draw do
  # ヘルスチェック用（最初からあったもの）
  get "up" => "rails/health#show", as: :rails_health_check

  # APIの定義
  namespace :api do
    namespace :v1 do
      # /api/v1/wallets/:id/withdraw というURLを作成します
      resources :wallets, only: [] do
        member do
          post :withdraw
        end
      end
    end
  end
end