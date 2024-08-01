package j.bindgen.api;

import io.quarkus.elytron.security.common.BcryptUtil;
import io.quarkus.security.UnauthorizedException;
import j.bindgen.api.dto.LoginDto;
import j.bindgen.api.dto.RegisterDto;
import j.bindgen.api.dto.UserDto;
import j.bindgen.model.User;
import j.bindgen.service.JwtService;
import jakarta.enterprise.inject.UnproxyableResolutionException;
import jakarta.inject.Inject;
import jakarta.validation.Valid;
import jakarta.ws.rs.Consumes;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Response;

@Path("/auth")
@Consumes(MediaType.APPLICATION_JSON)
@Produces(MediaType.APPLICATION_JSON)
public class AuthController {

    @Inject
    JwtService jwtService;

    @POST
    @Path("login")
    public Response login(@Valid LoginDto dto) {
        var error = new UnauthorizedException(("Invalid email or password"));
        var userOpt = User.findByEmail(dto.email);
        var user = userOpt.orElseThrow(() -> error);
        if (BcryptUtil.matches(dto.password, user.password)) {
            var userDto = UserDto.from(user);
            return jwtService.intoResponse(userDto);
        } else {
            throw error;
        }
    }

    @POST
    @Path("register")
    public Response registerUser(@Valid RegisterDto dto) {
        var userOpt = User.findByEmail(dto.email);

        if (userOpt.isPresent()) {
            throw new UnproxyableResolutionException(("Email is taken"));
        }

        var user = User.insertUser(dto.email, dto.password);
        var userDto = UserDto.from(user);
        return jwtService.intoResponse(userDto);
    }
}
