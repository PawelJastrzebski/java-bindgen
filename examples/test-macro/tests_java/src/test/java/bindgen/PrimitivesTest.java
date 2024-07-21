package bindgen;

import org.junit.jupiter.api.Test;

import com.test.macro.TestMacro;

import static org.junit.jupiter.api.Assertions.*;

public class PrimitivesTest {

    @Test
    public void returns_void() {
        TestMacro.returns_void();
    }

    @Test
    public void returns_jshort() {
        short s = TestMacro.returns_jshort();
        assertEquals(16, s);
    }

    @Test
    public void returns_jshort_i16() {
        short s = TestMacro.returns_jshort_i16();
        assertEquals(17, s);
    }

    @Test
    public void returns_jint() {
        int i = TestMacro.returns_jint();
        assertEquals(32, i);
    }

    @Test
    public void returns_jint_i32() {
        int i = TestMacro.returns_jint_i32();
        assertEquals(32, i);
    }

    @Test
    public void returns_jlong() {
        long l = TestMacro.returns_jlong();
        assertEquals(64L, l);
    }

    @Test
    public void returns_jlong_i64() {
        long l = TestMacro.returns_jlong_i64();
        assertEquals(64L, l);
    }

    @Test
    public void returns_jbyte() {
        byte b = TestMacro.returns_jbyte();
        assertEquals(8, b);
    }

    @Test
    public void returns_jbyte_u8() {
        byte b = TestMacro.returns_jbyte_u8();
        assertEquals(8, b);
    }

    @Test
    public void returns_jfloat() {
        float f = TestMacro.returns_jfloat();
        assertEquals(32.0f, f, 0.0);
    }

    @Test
    public void returns_jfloat_f32() {
        float f = TestMacro.returns_jfloat_f32();
        assertEquals(32.0f, f, 0.0);
    }

    @Test
    public void returns_jdouble() {
        double d = TestMacro.returns_jdouble();
        assertEquals(64.0, d, 0.0);
    }

    @Test
    public void returns_jdouble_i64() {
        double d = TestMacro.returns_jdouble_i64();
        assertEquals(64.0, d, 0.0);
    }

    @Test
    public void returns_boolean_bool() {
        boolean b = TestMacro.returns_boolean_bool();
        assertTrue(b);
    }

    @Test
    public void returns_jchar() {
        char c = TestMacro.returns_jchar();
        assertEquals('y', c);
    }

    @Test
    public void returns_jchar_char() {
        char c = TestMacro.returns_jchar_char();
        assertEquals('y', c);
    }

    @Test
    public void returns_string() {
        String s = TestMacro.returns_string();
        assertEquals("ok string", s);
    }

    @Test
    public void returns_byte_array() {
        byte[] array = TestMacro.returns_byte_array();
        assertArrayEquals(new byte[] { 1, 2, 3 }, array);
    }

    @Test
    public void returns_JByte() {
        Byte b = TestMacro.returns_JByte();
        assertEquals(Byte.valueOf((byte) 2), b);
    }

    @Test
    public void returns_JShort() {
        Short b = TestMacro.returns_JShort();
        assertEquals(Short.valueOf((short) 3), b);
    }

    @Test
    public void returns_JInt() {
        Integer b = TestMacro.returns_JInt();
        assertEquals(Integer.valueOf(4), b);
    }

    @Test
    public void returns_JLong() {
        Long b = TestMacro.returns_JLong();
        assertEquals(Long.valueOf(4L), b);
    }

    @Test
    public void returns_JFloat() {
        Float b = TestMacro.returns_JFloat();
        assertEquals(Float.valueOf(5.0f), b);
    }

    @Test
    public void returns_JDouble() {
        Double b = TestMacro.returns_JDouble();
        assertEquals(Double.valueOf(6.0f), b);
    }

    @Test
    public void returns_JBoolean() {
        Boolean b = TestMacro.returns_JBoolean();
        assertEquals(true, b);
    }

    @Test
    public void returns_JChar() {
        Character b = TestMacro.returns_JChar();
        assertEquals(Character.valueOf('y'), b);
    }

    // Input types

    @Test
    public void input_u8() {
        TestMacro.input_u8((byte) 8);
    }

    @Test
    public void input_i16() {
        TestMacro.input_i16((short) 16);
    }

    @Test
    public void input_i32() {
        TestMacro.input_i32(32);
    }

    @Test
    public void input_i64() {
        TestMacro.input_i64(64L);
    }

    @Test
    public void input_f32() {
        TestMacro.input_f32(32.0f);
    }

    @Test
    public void input_f64() {
        TestMacro.input_f64(64.0);
    }

    @Test
    public void input_string() {
        TestMacro.input_string("test");
    }

    @Test
    public void input_byte_array() {
        TestMacro.input_byte_array(new byte[] { 1, 2, 3 });
    }

    // Pass Types

    @Test
    public void pass_u8() {
        assertEquals((byte) 8, TestMacro.pass_u8((byte) 8));
        assertEquals((byte) -8, TestMacro.pass_u8((byte) -8));
        assertEquals(Byte.MAX_VALUE, TestMacro.pass_u8(Byte.MAX_VALUE));
        assertEquals(Byte.MIN_VALUE, TestMacro.pass_u8(Byte.MIN_VALUE));
    }

    @Test
    public void pass_i8() {
        assertEquals((byte) 9, TestMacro.pass_i8((byte) 9));
        assertEquals((byte) -9, TestMacro.pass_i8((byte) -9));
        assertEquals(Byte.MAX_VALUE, TestMacro.pass_i8(Byte.MAX_VALUE));
        assertEquals(Byte.MIN_VALUE, TestMacro.pass_i8(Byte.MIN_VALUE));
    }

    @Test
    public void pass_i16() {
        assertEquals(16, TestMacro.pass_i16((short) 16));
        assertEquals(Short.MAX_VALUE, TestMacro.pass_i16(Short.MAX_VALUE));
        assertEquals(Short.MIN_VALUE, TestMacro.pass_i16(Short.MIN_VALUE));
    }

    @Test
    public void pass_i32() {
        assertEquals(32, TestMacro.pass_i32(32));
        assertEquals(Short.MAX_VALUE, TestMacro.pass_i32(Short.MAX_VALUE));
        assertEquals(Short.MIN_VALUE, TestMacro.pass_i32(Short.MIN_VALUE));
    }

    @Test
    public void pass_i64() {
        assertEquals(64L, TestMacro.pass_i64(64L));
        assertEquals(Long.MAX_VALUE, TestMacro.pass_i64(Long.MAX_VALUE));
        assertEquals(Long.MIN_VALUE, TestMacro.pass_i64(Long.MIN_VALUE));
    }

    @Test
    public void pass_f32() {
        assertEquals(32.0f, TestMacro.pass_f32(32.0f));
        assertEquals(Float.MAX_VALUE, TestMacro.pass_f32(Float.MAX_VALUE));
        assertEquals(Float.MIN_VALUE, TestMacro.pass_f32(Float.MIN_VALUE));
    }

    @Test
    public void pass_f64() {
        assertEquals(64.0, TestMacro.pass_f64(64.0));
        assertEquals(Double.MAX_VALUE, TestMacro.pass_f64(Double.MAX_VALUE));
        assertEquals(Double.MIN_VALUE, TestMacro.pass_f64(Double.MIN_VALUE));
    }

    @Test
    public void pass_string() {
        assertEquals("test", TestMacro.pass_string("test"));
    }

    @Test
    public void pass_byte_array() {
        byte[] input = new byte[] { 1, 2, -3, Byte.MAX_VALUE, Byte.MIN_VALUE};
        assertArrayEquals(input, TestMacro.pass_byte_array(input));
    }

}
