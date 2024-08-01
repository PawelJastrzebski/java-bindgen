package j.bindgen.security;

import io.quarkus.security.credential.Credential;
import io.quarkus.security.identity.CurrentIdentityAssociation;
import io.quarkus.security.identity.SecurityIdentity;
import io.smallrye.mutiny.Uni;
import j.bindgen.api.dto.UserDto;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;
import lombok.RequiredArgsConstructor;
import org.jboss.logging.Logger;

import java.security.Permission;
import java.security.Principal;
import java.util.Map;
import java.util.Optional;
import java.util.Set;

@RequiredArgsConstructor
@ApplicationScoped
public class AuthUser {

    @Inject
    CurrentIdentityAssociation identity;

    Logger logger = Logger.getLogger(AuthUser.class);

    public Optional<UserDto> getUser() {
        if (identity.getIdentity() instanceof Auth auth) {
            return Optional.of(auth.user);
        }
        return Optional.empty();
    }

    public static class Auth implements SecurityIdentity {
        public UserDto user;

         Auth(UserDto user) {
            this.user = user;
        }

        @Override
        public Principal getPrincipal() {
            return null;
        }

        @Override
        public boolean isAnonymous() {
            return false;
        }

        @Override
        public Set<String> getRoles() {
            return Set.of();
        }

        @Override
        public boolean hasRole(String s) {
            // todo split ','
            return user.role.contains(s);
        }

        @Override
        public <T extends Credential> T getCredential(Class<T> aClass) {
            return null;
        }

        @Override
        public Set<Credential> getCredentials() {
            return Set.of();
        }

        @Override
        public <T> T getAttribute(String s) {
            return null;
        }

        @Override
        public Map<String, Object> getAttributes() {
            return Map.of();
        }

        @Override
        public Uni<Boolean> checkPermission(Permission permission) {
            return Uni.createFrom().item(false);
        }

    }


}
