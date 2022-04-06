using Benchmark.Dotnet.Models;

var builder = WebApplication.CreateBuilder(args);

var app = builder.Build();

app.MapGet("/api/user", () => 
{
    return new User(Guid.NewGuid(), "Benjamin", "HEINTZ", "heintz.benjamin@gmail.com");
});

app.Run();
