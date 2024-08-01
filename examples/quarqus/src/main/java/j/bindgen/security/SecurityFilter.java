package j.bindgen.security;


import io.quarkus.security.identity.CurrentIdentityAssociation;
import io.smallrye.mutiny.Uni;
import j.bindgen.service.JwtService;
import jakarta.annotation.Priority;
import jakarta.inject.Inject;
import jakarta.ws.rs.container.ContainerRequestContext;
import jakarta.ws.rs.container.ContainerRequestFilter;
import jakarta.ws.rs.container.PreMatching;

import java.io.IOException;

import jakarta.ws.rs.ext.Provider;
import lombok.SneakyThrows;
import org.eclipse.microprofile.jwt.JsonWebToken;
import org.jboss.logging.Logger;

@Provider
@Priority(1)
@PreMatching
public class SecurityFilter implements ContainerRequestFilter {

    Logger logger = Logger.getLogger(SecurityFilter.class);

    @Inject
    CurrentIdentityAssociation identity;

    @Inject
    AuthUser authUser;

    @Inject
    JsonWebToken jwt;

    @Inject
    JwtService jwtService;

    @Override
    @SneakyThrows
    public void filter(ContainerRequestContext req) throws IOException {

        // JWT Auth
        String token = req.getHeaders().getFirst("Authorization");
        if (token != null && token.toLowerCase().contains("bearer")) {
            var jwtToken = jwt.getRawToken();
            var user = jwtService.decode(jwtToken);
            var authUser = new AuthUser.Auth(user);
            identity.setIdentity(authUser);
        }

    }
}
