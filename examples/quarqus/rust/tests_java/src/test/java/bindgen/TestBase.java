package bindgen;

import image.rs.ImgProcessingRust;
import lombok.SneakyThrows;
import org.junit.jupiter.api.Test;

import java.io.File;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.assertNotNull;

public class TestBase {

    @Test
    public void should_load_lib() {
        ImgProcessingRust.loadNativeLibrary();
        assertNotNull(ImgProcessingRust.libVersion);
    }

    @SneakyThrows
    public static byte[] fetch_resource(String path) {
        ClassLoader classloader = Thread.currentThread().getContextClassLoader();
        byte[] bytes;
        try (InputStream br = classloader.getResourceAsStream(path)) {
            assert br != null;
            byte[] buffer = new byte[br.available()];
            int bytesRead = br.read(buffer);
            bytes = buffer;

        }
        return bytes;
}

    @SneakyThrows
    public void save_image(byte[] img, String path) {
        var resultsDir = new File("test_results");
        var created = resultsDir.mkdirs();

        Path filePath = Path.of("test_results", path);
        FileOutputStream outputStream = new FileOutputStream(filePath.toFile());
        outputStream.write(img);
        outputStream.close();
    }
}
