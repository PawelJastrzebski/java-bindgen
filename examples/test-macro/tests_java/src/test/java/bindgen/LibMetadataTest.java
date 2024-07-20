package bindgen;

import static org.junit.jupiter.api.Assertions.assertNotNull;

import org.junit.jupiter.api.Test;

import com.test.macro.TestMacro;

public class LibMetadataTest {

    @Test
    public void should_get_lib_metadata() {
        assertNotNull(TestMacro.libName);
        assertNotNull(TestMacro.libVersion);
        assertNotNull(TestMacro.libRelease);

        assertNotNull(TestMacro.libPath);
        assertNotNull(TestMacro.libExtension);
        System.out.println("Release date: " + TestMacro.libRelease);
    }

    @Test
    public void should_not_load_multiple_times() {
        TestMacro.loadNativeLibrary();
        TestMacro.loadNativeLibrary();
        TestMacro.loadNativeLibrary();
    }

}
