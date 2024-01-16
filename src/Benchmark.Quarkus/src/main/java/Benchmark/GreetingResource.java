package Benchmark;

import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Response;
import java.util.UUID;

import Benchmark.models.User;

@Path("/api/user")
public class GreetingResource {

    @GET
    @Produces(MediaType.APPLICATION_JSON)
    public Response hello() {
        User user = new User(UUID.randomUUID(), "Benjamin", "HEINTZ", "heintz.benjamin@gmail.com");
        return Response.ok(user).build();
    }
}