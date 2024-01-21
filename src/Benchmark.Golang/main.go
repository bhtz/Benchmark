package main

import (
	"bhtz/Benchmark.Golang/models"
	"fmt"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

func main() {

	gin.SetMode(gin.ReleaseMode)
	r := gin.New()

	r.GET("/", func(c *gin.Context) {
		msg := map[string]interface{}{
			"name": "Benchmark.Golang",
		}
		c.JSON(200, msg)
	})

	r.GET("/api/user", func(c *gin.Context) {
		user := models.NewUser(uuid.NewString(), "Benjamin", "HEINTZ", "heintz.benjamin@gmail.com")
		c.JSON(200, user)
	})

	fmt.Printf("Server listening at port 8080 ...")

	r.Run()
}
