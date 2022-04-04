using Benchmark.Dotnet.Models;

var builder = WebApplication.CreateBuilder(args);

var app = builder.Build();

app.MapGet("/api/user", () => 
{
    return new User()
    {
        Id = Guid.NewGuid(),
        Firstname = "Benjamin",
        Lastname = "HEINTZ",
        Email = "heintz.benjamin@gmail.com"
    };
});

app.Run();
