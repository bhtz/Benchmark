package bhtz.Benchmark;

import java.util.UUID;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;
import java.util.UUID;

import bhtz.Benchmark.models.User;

@SpringBootApplication
@RestController
public class JavaApplication {

    public static void main(String[] args) {
        SpringApplication.run(JavaApplication.class, args);
    }

    @GetMapping("/api/user")
    public User getUser() {
        User user = new User(UUID.randomUUID(), "Benjamin", "HEINTZ", "heintz.benjamin@gmail.com");
        return user;
    }
}
