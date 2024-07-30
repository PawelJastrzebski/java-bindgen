package j.bindgen.api;

import io.quarkus.test.junit.QuarkusTest;
import j.bindgen.TestBase;
import j.bindgen.api.dto.LoginDto;
import j.bindgen.api.dto.RegisterDto;
import j.bindgen.model.User;
import org.junit.jupiter.api.Test;

import java.util.UUID;

import static org.assertj.core.api.AssertionsForClassTypes.assertThat;

@QuarkusTest
class AuthControllerTest extends TestBase {

    @Test
    void should_login() {
        anonymous()
                .when()
                .body(LoginDto.builder().email("admin").password("admin").build())
                .header("Content-Type", "application/json")
                .post("/auth/login")
                .then()
                .statusCode(200);
    }

    @Test
    void should_not_login() {
        anonymous()
                .when()
                .body(LoginDto.builder().email("admin").password("INVALID_PASSWORD").build())
                .header("Content-Type", "application/json")
                .post("/auth/login")
                .then()
                .statusCode(401);
    }

    @Test
    void should_register() {
        var email = "login_" + UUID.randomUUID();
        var password = "password_" + UUID.randomUUID();
        anonymous()
                .when()
                .body(RegisterDto.builder().email(email).password(password).build())
                .header("Content-Type", "application/json")
                .post("/auth/register")
                .then()
                .statusCode(200);

        var dbUser = User.findByEmail(email);
        assertThat(dbUser).isNotEmpty();
        assertThat(dbUser.get().email).isEqualTo(email);
        assertThat(dbUser.get().password).isNotEqualTo(password);
    }

}