package bindgen;

import com.test.macro.JavaClassWrappers;
import org.junit.jupiter.api.Test;

import com.test.macro.AllJavaTypes;
import com.test.macro.TestMacro;
import com.test.macro.UserClass;

import static org.junit.jupiter.api.Assertions.*;

public class CustomTypesTest {


    @Test
    public void get_user() {
        UserClass user = TestMacro.get_user();
        assertEquals("Tom", user.getName());
    }

    @Test
    public void pass_user() {
        UserClass user = TestMacro.pass_user("Hello, ", new UserClass("Java", 2));
        System.out.println(user);
        assertEquals("Hello, Java", user.getName());
        assertEquals(102, user.getAge());
    }

    @Test
    public void pass_all_types() {
        byte java_b = (byte) -7;
        short java_s = 2;
        int java_i = 12;
        long java_l = 23L;
        float java_f = 30.0f;
        double java_d = 40.0;
        char java_c = 'a';
        boolean java_bool = true;
        String java_String = "ok";
        byte[] java_b_array = new byte[] { 4, 5, 6 };

        AllJavaTypes all = AllJavaTypes
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

        AllJavaTypes res = TestMacro.pass_all_types(all);
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

    @Test
    public void pass_java_class_wrappers() {
        Byte java_b = (byte) -7;
        Short java_s = 2;
        Integer java_i = 12;
        Long java_l = 23L;
        Float java_f = 30.0f;
        Double java_d = 40.0;
        Character java_c = 'a';
        Boolean java_bool = true;

        JavaClassWrappers all = JavaClassWrappers
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

        JavaClassWrappers res = TestMacro.pass_java_class_wrappers(all);
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

}
