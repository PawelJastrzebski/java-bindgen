package bindgen;

import com.test.macro.TestMacro;
import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

public class PassListTest {

    @Test
    public void pass_list_u8() {
        List<Byte> input = Arrays.asList((byte) 2, (byte) 10);
        List<Byte> out = TestMacro.pass_list_u8(input);
        assertEquals(input.get(0), out.get(0));
        assertEquals(input.get(1), out.get(1));
    }

    @Test
    public void pass_list_i8() {
        List<Byte> input = Arrays.asList((byte) 2, (byte) 10);
        List<Byte> out = TestMacro.pass_list_i8(input);
        assertEquals(input.get(0), out.get(0));
        assertEquals(input.get(1), out.get(1));
    }

    @Test
    public void pass_list_i16() {
        List<Short> input = Arrays.asList((short) 2, (short) 10);
        List<Short> out = TestMacro.pass_list_i16(input);
        assertEquals(input.get(0), out.get(0));
        assertEquals(input.get(1), out.get(1));
    }

    @Test
    public void pass_list_i32() {
        List<Integer> input = Arrays.asList(2, 10);
        List<Integer> out = TestMacro.pass_list_i32(input);
        assertEquals(input.get(0), out.get(0));
        assertEquals(input.get(1), out.get(1));
    }

    @Test
    public void pass_list_i64() {
        List<Long> input = Arrays.asList(2L, 10L);
        List<Long> out = TestMacro.pass_list_i64(input);
        assertEquals(input.get(0), out.get(0));
        assertEquals(input.get(1), out.get(1));
    }

    @Test
    public void pass_list_f32() {
        List<Float> input = Arrays.asList(2.0f, 10.0f);
        List<Float> out = TestMacro.pass_list_f32(input);
        assertEquals(input.get(0), out.get(0));
        assertEquals(input.get(1), out.get(1));
    }

    @Test
    public void pass_list_f64() {
        List<Double> input = Arrays.asList(2.0, 10.0);
        List<Double> out = TestMacro.pass_list_f64(input);
        assertEquals(input.get(0), out.get(0));
        assertEquals(input.get(1), out.get(1));
    }

    @Test
    public void pass_list_char() {
        List<Character> input = Arrays.asList('o', 'w');
        List<Character> out = TestMacro.pass_list_char(input);
        assertEquals(input.get(0), out.get(0));
        assertEquals(input.get(1), out.get(1));
    }

    @Test
    public void pass_list_bool() {
        List<Boolean> input = Arrays.asList(true, false);
        List<Boolean> out = TestMacro.pass_list_bool(input);
        assertEquals(input.get(0), out.get(0));
        assertEquals(input.get(1), out.get(1));
    }

    @Test
    public void pass_list_string() {
        List<String> input = Arrays.asList("ok", "doki");
        List<String> out = TestMacro.pass_list_string(input);
        assertEquals(input.get(0), out.get(0));
        assertEquals(input.get(1), out.get(1));
    }

    @Test
    public void pass_list_byte_array() {
        List<byte[]> input = Arrays.asList(new byte[] { 12, 2, 123 }, new byte[] { -126, 23, -43 });
        List<byte[]> out = TestMacro.pass_list_byte_array(input);
        assertArrayEquals(input.get(0), out.get(0));
        assertArrayEquals(input.get(1), out.get(1));
    }

    @Test
    public void pass_list_of_lists() {
        List<List<Integer>> input = Arrays.asList(
                Arrays.asList(3, 4),
                Arrays.asList(2, 3)
        );
        List<List<Integer>> out = TestMacro.pass_list_of_lists(input);
        System.out.println(out);
        assertEquals(input.get(0), out.get(0));
        assertEquals(input.get(1), out.get(1));
    }


}
