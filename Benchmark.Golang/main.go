package main

import (
	"bhtz/Benchmark.Golang/models"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

func main() {

	r := gin.Default()

	r.GET("/api/user", func(c *gin.Context) {
		user := models.User{
			Id:        uuid.NewString(),
			Email:     "heintz.benjamin@gmail.com",
			Firstname: "Benjamin",
			Lastname:  "HEINTZ",
		}

		c.JSON(200, user)
	})

	r.Run(":5100")
}
