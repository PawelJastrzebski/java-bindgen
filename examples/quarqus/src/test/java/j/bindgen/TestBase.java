package j.bindgen;

import io.restassured.specification.RequestSpecification;
import j.bindgen.api.dto.UserDto;
import j.bindgen.model.User;
import j.bindgen.service.JwtService;
import jakarta.inject.Inject;
import lombok.RequiredArgsConstructor;
import org.junit.jupiter.api.Test;

import static io.restassured.RestAssured.given;

@RequiredArgsConstructor
public class TestBase {

    @Inject
    public JwtService jwtService;

    @Test
    public void empty_test(){}

    public RequestSpecification authorized() {
       var admin = User.findByEmail("admin").orElseThrow();
       var jwt = jwtService.encode(UserDto.from(admin));
        return given().header("Authorization", "Bearer " + jwt);
    }

    public RequestSpecification anonymous() {
        return given();
    }
}
