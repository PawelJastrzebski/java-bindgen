package j.bindgen.service;

import io.quarkus.test.junit.QuarkusTest;
import io.smallrye.jwt.auth.principal.ParseException;
import j.bindgen.api.dto.UserDto;
import jakarta.inject.Inject;
import lombok.extern.java.Log;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

@Log
@QuarkusTest
class JwtServiceTest {

    @Inject
    JwtService jwtService;

    @Test
    public void should_encode_user() {
        var jwt = jwtService.encode(UserDto.builder().id(2).email("test@gmail.com").role("OK").build());
        assertThat(jwt).isNotBlank();
        log.info("Jwt: " + jwt);
    }

    @Test
    public void should_decode_user() throws ParseException {
        var test_user = UserDto.builder().id(2).email("test@gmail.com").role("OK").build();
        var jwt = jwtService.encode(test_user);

        var user = jwtService.decode(jwt);
        assertThat(user).isNotNull();
        assertThat(user.id).isEqualTo(test_user.id);
        assertThat(user.email).isEqualTo(test_user.email);
        assertThat(user.role).isEqualTo(test_user.role);
    }

}