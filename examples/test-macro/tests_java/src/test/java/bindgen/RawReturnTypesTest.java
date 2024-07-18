package bindgen;

import com.test.macro.EmptyClass;
import org.junit.jupiter.api.Test;

import com.test.macro.TestMacro;

import static org.junit.jupiter.api.Assertions.*;

public class RawReturnTypesTest {

    @Test
    public void raw_return_object() {
        EmptyClass s = TestMacro.raw_return_object();
        assertNotNull(s);
    }

    @Test
    public void raw_return_string_1() {
        String s = TestMacro.raw_return_string_1();
        assertEquals("Hello", s);
    }

    @Test
    public void raw_return_string_2() {
        String s = TestMacro.raw_return_string_2();
        assertEquals("Hello", s);
    }

    @Test
    public void raw_return_bytes() {
        byte[] b = TestMacro.raw_return_bytes(new byte[] { 1, 21, 34 });
        assertArrayEquals(new byte[] { 1, 21, 34 }, b);
    }

}
