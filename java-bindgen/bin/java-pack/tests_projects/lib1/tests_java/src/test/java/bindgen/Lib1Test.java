package bindgen;

import com.test.Lib1;
import com.test.UserClass;

import org.junit.jupiter.api.Test;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import static org.junit.jupiter.api.Assertions.*;

public class Lib1Test {

    public static final Logger logger = LoggerFactory.getLogger(Lib1Test.class);

    @Test
    public void should_get_lib_metadata() {
        assertNotNull(Lib1.libName);
        assertNotNull(Lib1.libVersion);
        assertNotNull(Lib1.libRelease);
        
        assertNotNull(Lib1.libPath);
        assertNotNull(Lib1.libExtension);
        System.out.println("Release date: " + Lib1.libRelease);
    }

    @Test
    public void should_throw_exception() {
        assertThrows(RuntimeException.class, () -> {
            Lib1.ethrow();
        });
    }

    @Test
    public void should_return_JavaClass() {
        UserClass u = Lib1.user();
        System.out.println(u);
        assertNotNull(u);
        assertEquals("Hello", u.getName());
        assertEquals(220, u.getAge());
        assertEquals(3, u.getArray().length);
    }

    @Test
    public void should_call_defined_methods() {
        String msg = Lib1.hello("ok");
        assertEquals("Hello Java java_bindgen 4444 FullAuto, ok!", msg);
        
        byte[] array = Lib1.helloByte(new byte[1]);
        assertEquals(2000, array.length);

        byte b = Lib1.user1();
        assertEquals(127, b);
    }

}

