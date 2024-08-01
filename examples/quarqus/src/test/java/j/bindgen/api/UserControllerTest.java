package j.bindgen.api;

import io.quarkus.test.junit.QuarkusTest;
import j.bindgen.TestBase;
import org.junit.jupiter.api.Test;

@QuarkusTest
class UserControllerTest extends TestBase {

    @Test
    void get_user_anonymous() {
        anonymous()
                .when().get("/user")
                .then()
                .statusCode(401);

    }

    @Test
    void get_user_authorized() {
        authorized()
                .when().get("/user")
                .then()
                .statusCode(200);

    }

}