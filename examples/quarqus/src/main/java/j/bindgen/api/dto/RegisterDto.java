package j.bindgen.api.dto;

import jakarta.validation.constraints.NotBlank;

public class RegisterDto {
    @NotBlank(message = "Email is required")
    public String email;
    @NotBlank(message = "Password is required")
    public String password;
}
