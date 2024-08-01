package j.bindgen.api.dto;

import j.bindgen.model.User;
import lombok.Builder;
import lombok.ToString;

@Builder
@ToString
public class UserDto {
    public int id;
    public String email;
    public String role;

    public static UserDto from(User user) {
        return UserDto.builder()
                .id(user.id)
                .email(user.email)
                .role(user.role)
                .build();
    }
}
