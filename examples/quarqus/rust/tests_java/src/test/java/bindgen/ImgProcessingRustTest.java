package bindgen;

import image.rs.ImageInput;
import image.rs.ImgProcessingRust;
import image.rs.TransformResult;
import org.junit.jupiter.api.Test;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Collections;

public class ImgProcessingRustTest extends TestBase {

    public static final Logger log = LoggerFactory.getLogger(ImgProcessingRustTest.class);

    static byte[] TEST_IMG = fetch_resource("images/StockSnap_KAUFJW1PEQ_HD.jpg");

    @Test
    public void process_no_transform() {
        ImageInput img = ImageInput.builder().image(TEST_IMG).ext("jpg").build();
        TransformResult result = ImgProcessingRust.processImage(img, Collections.emptyList());
        save_image(result.getImage(), "result_unchanged.png");
    }

    @Test
    public void process_resize_200x300() {
        ImageInput img = ImageInput.builder().image(TEST_IMG).ext("jpg").build();
        TransformResult result = ImgProcessingRust.processImage(img, Collections.singletonList("resize:300,300"));
        save_image(result.getImage(), "result_300x300.png");
    }

    @Test
    public void process_resize_100x300() {
        ImageInput img = ImageInput.builder().image(TEST_IMG).ext("jpg").build();
        TransformResult result = ImgProcessingRust.processImage(img, Collections.singletonList("resize:100,300"));
        save_image(result.getImage(), "result_100x300.png");
    }

    @Test
    public void process_resize_300x100() {
        ImageInput img = ImageInput.builder().image(TEST_IMG).ext("jpg").build();
        TransformResult result = ImgProcessingRust.processImage(img, Collections.singletonList("resize:300,100"));
        save_image(result.getImage(), "result_300x100.png");
    }

    @Test
    public void process_contrast_1() {
        ImageInput img = ImageInput.builder().image(TEST_IMG).ext("jpg").build();
        TransformResult result = ImgProcessingRust.processImage(img, Collections.singletonList("contrast:1.0"));
        save_image(result.getImage(), "result_contrast_1.0.png");
    }

    @Test
    public void process_contrast_minus_1() {
        ImageInput img = ImageInput.builder().image(TEST_IMG).ext("jpg").build();
        TransformResult result = ImgProcessingRust.processImage(img, Collections.singletonList("contrast:-1.0"));
        save_image(result.getImage(), "result_contrast_-1.0.png");
        log.info("transforms: {}", result.getAppliedTransforms());
    }

    @Test
    public void process_contrast_0() {
        ImageInput img = ImageInput.builder().image(TEST_IMG).ext("jpg").build();
        TransformResult result = ImgProcessingRust.processImage(img, Collections.singletonList("contrast:0"));
        save_image(result.getImage(), "result_contrast_0.png");
        log.info("transforms: {}", result.getAppliedTransforms());
    }
}

