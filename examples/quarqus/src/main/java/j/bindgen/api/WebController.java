package j.bindgen.api;

import jakarta.ws.rs.Consumes;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Response;
import org.jboss.logging.Logger;

import java.net.URI;

@Path("/")
public class WebController {

    Logger logger = Logger.getLogger(WebController.class);

    @GET
    @Path("/app")
    public Response redirect_to_web_app() {
        // Redirect to Web /app/*
        return Response.temporaryRedirect(URI.create("/app/")).build();
    }

    @GET
    @Consumes(MediaType.TEXT_HTML)
    public Response redirect_to_app() {
        return this.redirect_to_web_app();
    }
}
