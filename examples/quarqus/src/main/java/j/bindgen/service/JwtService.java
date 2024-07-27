package j.bindgen.service;

import com.google.gson.Gson;
import io.smallrye.jwt.auth.principal.DefaultJWTTokenParser;
import io.smallrye.jwt.auth.principal.JWTAuthContextInfo;
import io.smallrye.jwt.auth.principal.ParseException;
import io.smallrye.jwt.build.Jwt;
import io.smallrye.jwt.util.KeyUtils;
import j.bindgen.api.dto.UserDto;
import j.bindgen.security.JwtParser;
import jakarta.annotation.PostConstruct;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.ws.rs.core.Response;
import lombok.extern.java.Log;
import org.eclipse.microprofile.config.inject.ConfigProperty;
import org.jboss.logging.Logger;
import org.jose4j.jwt.MalformedClaimException;
import org.jose4j.jwt.consumer.JwtContext;

import java.io.IOException;
import java.security.GeneralSecurityException;
import java.security.PrivateKey;
import java.security.PublicKey;
import java.time.Instant;
import java.time.temporal.ChronoUnit;


@Log
@ApplicationScoped
public class JwtService {

    Logger logger = Logger.getLogger(JwtParser.class);

    @ConfigProperty(name = "smallrye.jwt.sign.key.location")
    private String PRIVATE_KEY_LOCATION;
    @ConfigProperty(name = "mp.jwt.verify.publickey.location")
    private String PUB_KEY_LOCATION;

    private PublicKey PUB_KEY;
    private PrivateKey PRIVATE_KEY;

    public String loadKey(String location) throws IOException {
        try (var is = this.getClass().getResourceAsStream(location)) {
            assert is != null;
            byte[] bytes = is.readAllBytes();
            return new String(bytes);
        }
    }

    @PostConstruct
    public void init() throws IOException, GeneralSecurityException {
        var pubKey = loadKey(PUB_KEY_LOCATION);
        var privateKey = loadKey(PRIVATE_KEY_LOCATION);

        PRIVATE_KEY = KeyUtils.decodePrivateKey(privateKey);
        PUB_KEY = KeyUtils.decodePublicKey(pubKey);
    }

    public String encode(UserDto user) {
        var userJson = new Gson().toJson(user);
        var expiresAt = Instant.now().plus(45, ChronoUnit.MINUTES).getEpochSecond();

        return Jwt.claim("user", userJson)
                .subject("User")
                .issuer("java-pack.rs")
                .expiresAt(expiresAt)
                .sign(PRIVATE_KEY);
    }

    public UserDto decode(String token) throws ParseException {
        var jwt = this.validate(token);
        try {
            String userJson = jwt.getJwtClaims().getClaimValueAsString("user");
            return new Gson().fromJson(userJson, UserDto.class);
        } catch (Exception e) {
            logger.error(e);
            throw e;
        }
    }

    public JwtContext validate(String token) throws ParseException {
        var ctx = new JWTAuthContextInfo();
        ctx.setPublicVerificationKey(PUB_KEY);
        var parser = new DefaultJWTTokenParser();
        try {
            JwtContext jwt = parser.parse(token, ctx);
            jwt.getJwtClaims().getClaimValueAsString("user");

            var expiresAt  = jwt.getJwtClaims().getClaimValue("exp", Long.class);
            var expiresAtInstant = Instant.ofEpochSecond(expiresAt);
            if (expiresAtInstant.isBefore(Instant.now())) {
                throw new ParseException("Token expired");
            }
            return jwt;
        } catch (MalformedClaimException ex) {
            throw new ParseException("Invalid JWT format");
        } catch (Exception e) {
            logger.error(e);
            logger.error("JWT: " + token);
            throw e;
        }
    }

    public Response intoResponse(UserDto authUser) {
       return Response.ok(authUser).header("token", this.encode(authUser)).build();
    }

}
