FROM golang:1.21.6

WORKDIR /go/src/app

COPY . .

RUN go build -ldflags "-s -w" main.go
EXPOSE 5500

CMD ["./main"]