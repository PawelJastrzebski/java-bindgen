package bindgen;

import org.junit.jupiter.api.Test;

import com.test.macro.TestMacro;

import static org.junit.jupiter.api.Assertions.*;

public class RawInputTypeTest {

    @Test
    public void raw_input_type_1() {
        String s = TestMacro.raw_input_type_1("ok");
        assertEquals("ok", s);
    }

    @Test
    public void raw_input_type_2() {
        String s = TestMacro.raw_input_type_2("ok");
        assertEquals("ok", s);
    }

    @Test
    public void raw_input_type_2__pass_incorrect_object_type() {
        assertThrows(UnsupportedOperationException.class, () -> {
            try {
                String s = TestMacro.raw_input_type_2(new Integer(200));
                assertEquals("ok", s);
            }catch (UnsupportedOperationException e) {
                System.out.println(e);
                assertTrue(e.toString().contains("Rust"));
                assertTrue(e.toString().contains("Cast"));
                throw e;
            }
        });
    }

    @Test
    public void raw_input_type_3() {
        byte[] b = TestMacro.raw_input_type_3(new byte[] { 1, 21, 34 });
        assertArrayEquals(new byte[] { 1, 21, 34 }, b);
    }

}
