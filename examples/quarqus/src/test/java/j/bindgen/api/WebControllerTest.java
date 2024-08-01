package j.bindgen.api;

import io.quarkus.test.junit.QuarkusTest;
import org.junit.jupiter.api.Test;

import static io.restassured.RestAssured.given;
import static org.jboss.resteasy.reactive.RestResponse.StatusCode.TEMPORARY_REDIRECT;

@QuarkusTest
class WebControllerTest {

    @Test
    void redirect_to_web_app() {
        given()
                .redirects().follow(false)
                .when().get("/")
                .then()
                .statusCode(TEMPORARY_REDIRECT)
                .header("Location", "http://localhost:8081/app/");

        given()
                .redirects().follow(false)
                .when().get("/app")
                .then()
                .statusCode(TEMPORARY_REDIRECT)
                .header("Location", "http://localhost:8081/app/");

    }
}