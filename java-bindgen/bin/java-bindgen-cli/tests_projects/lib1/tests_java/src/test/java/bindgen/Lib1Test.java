package bindgen;

import com.test.Lib1;
import org.junit.jupiter.api.Test;

import java.util.Arrays;

import static org.junit.jupiter.api.Assertions.*;

public class Lib1Test {

    @Test
    public void your_first_test() {
        System.out.println(Lib1.hello("ok"));

        System.out.println(Arrays.toString(Lib1.helloByte(new byte[1])));
        System.out.println("byte:" + Lib1.user1());

        try {
            System.out.println(Lib1.ethrow());
        } catch(Exception e) {
            System.out.print("Expected: " + e);
        }

        Object u = Lib1.user();
        System.out.println(u == null);
        System.out.println(u);

        System.out.println(Lib1.libRelease);
    }

}

