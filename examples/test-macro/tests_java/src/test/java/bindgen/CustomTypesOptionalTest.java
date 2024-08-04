package bindgen;

import com.test.macro.*;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

public class CustomTypesOptionalTest {

    @Test
    public void pass_java_class_OptionValue() {
        OptionPrimitive input = new OptionPrimitive(20);
        OptionPrimitive result = TestMacro.pass_java_class_option(input);
        assertEquals(20, result.getId());
    }

    @Test
    public void pass_java_class_OptionValue2() {
        OptionClassWrapper input = new OptionClassWrapper(20);
        OptionClassWrapper result = TestMacro.pass_java_class_option2(input);
        assertEquals(20, result.getId());
    }

    @Test
    public void pass_java_class_OptionValue2_null() {
        OptionClassWrapper input = new OptionClassWrapper(null);
        OptionClassWrapper result = TestMacro.pass_java_class_option2(input);
        assertNull(result.getId());
    }

    @Test
    public void pass_option_all_primitives() {
        byte java_b = (byte) -7;
        short java_s = 2;
        int java_i = 12;
        long java_l = 23L;
        float java_f = 30.0f;
        double java_d = 40.0;
        char java_c = 'a';
        boolean java_bool = true;
        String java_String = "ok";
        byte[] java_b_array = new byte[]{4, 5, 6};
        {
            OptionAllPrimitive all = OptionAllPrimitive
                    .builder()
                    .java_b(java_b)
                    .java_s(java_s)
                    .java_i(java_i)
                    .java_l(java_l)
                    .java_f(java_f)
                    .java_d(java_d)
                    .java_c(java_c)
                    .java_bool(java_bool)
                    .java_string(java_String)
                    .java_barray(java_b_array)
                    .build();

            OptionAllPrimitive res = TestMacro.pass_option_all_primitives(all);
            System.out.println(res);

            assertEquals(java_b, res.getJava_b());
            assertEquals(java_s, res.getJava_s());
            assertEquals(java_i, res.getJava_i());
            assertEquals(java_l, res.getJava_l());
            assertEquals(java_f, res.getJava_f());
            assertEquals(java_d, res.getJava_d());
            assertEquals(java_c, res.getJava_c());
            assertEquals(java_bool, res.isJava_bool());
            assertEquals(java_String, res.getJava_string());
            assertArrayEquals(java_b_array, res.getJava_barray());
        }
        {
            OptionAllPrimitive all = OptionAllPrimitive
                    .builder()
                    .build();

            OptionAllPrimitive res = TestMacro.pass_option_all_primitives(all);
            System.out.println(res);

            assertEquals(0, res.getJava_b());
            assertEquals(0, res.getJava_s());
            assertEquals(0, res.getJava_i());
            assertEquals(0, res.getJava_l());
            assertEquals(0, res.getJava_f());
            assertEquals(0, res.getJava_d());
            assertEquals('\0', res.getJava_c());
            assertFalse(res.isJava_bool());
            assertNull(res.getJava_string());
            assertNull(res.getJava_barray());
        }
    }

    @Test
    public void pass_option_all_class_wrappers() {
        Byte java_b = (byte) -7;
        Short java_s = 2;
        Integer java_i = 12;
        Long java_l = 23L;
        Float java_f = 30.0f;
        Double java_d = 40.0;
        Character java_c = 'a';
        Boolean java_bool = true;

        {
            OptionClassWrappers all = OptionClassWrappers
                    .builder()
                    .java_b(java_b)
                    .java_s(java_s)
                    .java_i(java_i)
                    .java_l(java_l)
                    .java_f(java_f)
                    .java_d(java_d)
                    .java_c(java_c)
                    .java_bool(java_bool)
                    .build();

            OptionClassWrappers res = TestMacro.pass_option_all_class_wrappers(all);
            System.out.println(res);

            assertEquals(java_b, res.getJava_b());
            assertEquals(java_s, res.getJava_s());
            assertEquals(java_i, res.getJava_i());
            assertEquals(java_l, res.getJava_l());
            assertEquals(java_f, res.getJava_f());
            assertEquals(java_d, res.getJava_d());
            assertEquals(java_c, res.getJava_c());
            assertEquals(java_bool, res.getJava_bool());
        }
        {
            OptionClassWrappers all = OptionClassWrappers.builder().build();
            OptionClassWrappers res = TestMacro.pass_option_all_class_wrappers(all);
            System.out.println(res);

            assertNull(res.getJava_b());
            assertNull(res.getJava_s());
            assertNull(res.getJava_i());
            assertNull(res.getJava_l());
            assertNull(res.getJava_f());
            assertNull(res.getJava_d());
            assertNull(res.getJava_c());
        }
    }

    @Test
    public void pass_option_element() {
        {
            OptionElement ele = new OptionElement(3, new OptionNode(2));
            OptionElement res = TestMacro.pass_option_element(ele);
            assertNotNull(res.getId());
            assertNotNull(res.getParent());
            assertEquals(3, res.getId());
            assertEquals(2, res.getParent().getId());
        }
        {
            OptionElement ele = new OptionElement(null, new OptionNode(null));
            OptionElement res = TestMacro.pass_option_element(ele);
            assertNull(res.getId());
            assertNotNull(res.getParent());
            assertNull(res.getParent().getId());
        }
        {
            OptionElement ele = new OptionElement(null, null);
            OptionElement res = TestMacro.pass_option_element(ele);
            assertNull(res.getId());
            assertNull(res.getParent());
        }
    }

}