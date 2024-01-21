package Benchmark;

import Benchmark.models.HomeMessage;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Response;

@Path("/")
public class HomeResource {

    @GET
    @Produces(MediaType.APPLICATION_JSON)
    public Response home() {
        HomeMessage msg = new HomeMessage("Benchmark.Java");
        return Response.ok(msg).build();
    }
}