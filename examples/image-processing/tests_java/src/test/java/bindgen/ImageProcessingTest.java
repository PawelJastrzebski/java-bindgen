package bindgen;

import org.junit.jupiter.api.Test;

import rs.image.ImageProcessing;
import rs.image.ImgSize;

import java.io.*;

public class ImageProcessingTest {

    public byte[] fetch_resource(String path) throws IOException {
        ClassLoader classloader = Thread.currentThread().getContextClassLoader();
        byte[] bytes;
        try (InputStream br = classloader.getResourceAsStream(path)) {
            assert br != null;
            byte[] buffer = new byte[br.available() ];
            int bytesRead = br.read(buffer);
            bytes = buffer;

        }
        return bytes;
    }

    public void save_image(byte[] img, String path) throws IOException {
        FileOutputStream outputStream = new FileOutputStream(path);
        outputStream.write(img);
        outputStream.close();
    }

    @Test
    public void your_first_test() throws Exception {
        byte[] input = fetch_resource("images/StockSnap_KAUFJW1PEQ_HD.jpg");
        ImageProcessing.logger.info("Pass to Rust:");
        byte[] image = ImageProcessing.resizeImage(input, "jpg", "png", new ImgSize(200, 200));
        ImageProcessing.logger.info("Get From Rust:");
        save_image(image, "result_200x200.png");
    }

}

