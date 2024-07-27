package j.bindgen.security;

import io.smallrye.jwt.auth.principal.*;
import j.bindgen.service.JwtService;
import jakarta.annotation.Priority;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.enterprise.inject.Alternative;
import jakarta.inject.Inject;
import org.jboss.logging.Logger;


@ApplicationScoped
@Alternative
@Priority(1)
public class JwtParser extends JWTCallerPrincipalFactory {

    Logger logger = Logger.getLogger(JwtParser.class);

    @Inject
    JwtService jwtService;

    @Override
    public JWTCallerPrincipal parse(String jwt, JWTAuthContextInfo jwtAuthContextInfo) throws ParseException {
        var jwt_ctx = jwtService.validate(jwt);
        return new DefaultJWTCallerPrincipal(jwt_ctx.getJwtClaims());
    }
}
