package main

import (
	"bhtz/Benchmark.Golang/models"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

func main() {

	r := gin.Default()

	r.GET("/api/user", func(c *gin.Context) {
		user := models.NewUser(uuid.NewString(), "Benjamin", "HEINTZ", "heintz.benjamin@gmail.com")
		c.JSON(200, user)
	})

	r.Run(":5100")
}
