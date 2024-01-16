namespace Benchmark.Dotnet.Models;

public class User
{
    public Guid Id { get; set; }
    public string Firstname { get; set; }
    public string Lastname { get; set; }
    public string Email { get; set; }

    public User(Guid id, string firstname, string lastname, string email)
    {
        Id = id;
        Firstname = firstname;
        Lastname = lastname;
        Email = email;
    }
}
