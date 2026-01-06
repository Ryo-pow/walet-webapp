package main

import (
	"math/rand"
	"net/http"

	"github.com/gin-gonic/gin"
)

func main() {
	r := gin.Default()

	r.GET("price", func(c *gin.Context) {
		basePrice := 500000.0
		randomChange := float64(rand.Intn(201) - 100) // -100 ~ +100

		c.JSON(http.StatusOK, gin.H{
			"symbol": "BTC",
			"price":  basePrice + randomChange,
		})
	})

	r.Run(":8080")
}
