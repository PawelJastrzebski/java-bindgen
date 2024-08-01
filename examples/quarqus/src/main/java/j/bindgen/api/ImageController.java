package j.bindgen.api;

import image.rs.ImageInput;
import image.rs.ImgProcessingRust;
import image.rs.TransformResult;
import j.bindgen.api.dto.ImageProcess;
import jakarta.validation.Valid;
import jakarta.ws.rs.Consumes;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Response;
import org.jboss.logging.Logger;

import java.util.Arrays;
import java.util.List;

@Path("/img")
public class ImageController {

    Logger logger = Logger.getLogger(UserController.class);

    @POST
    @Path("process")
    @Consumes(MediaType.MULTIPART_FORM_DATA)
    @Produces(MediaType.APPLICATION_JSON)
    public Response process(@Valid ImageProcess dto) {
        logger.info("data.image: " + dto.image.length);
        logger.info("data.imageExtension: " + dto.imageExtension);
        logger.info("data.transforms: " + dto.transforms);

        ImageInput img = ImageInput.builder().image(dto.image).ext(dto.imageExtension).build();
        List<String> transforms = Arrays.stream(dto.transforms.split(",")).toList();
        TransformResult result = ImgProcessingRust.processImage(img, transforms);

        logger.info("done: " + result.getImage().length);
        return Response.ok(result.getImage())
                .header("content-type", "image/" + result.getExt())
                .header("content-disposition", "attachment; filename=image." + result.getExt())
                .build();
    }

}
