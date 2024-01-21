using Benchmark.Dotnet.Models;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.MapGet("/", () =>
{
    return new { Name = "Benchmark.Dotnet" };
});

app.MapGet("/api/user", () =>
{
    return new User(Guid.NewGuid(), "Benjamin", "HEINTZ", "heintz.benjamin@gmail.com");
});

app.Run();
