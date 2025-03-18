use dioxus_translate::translate;

translate! {
    LoginTranslate;

    login: {
        ko: "로그인",
        en: "Login"
    },
    email: {
        ko: "이메일",
        en: "Email"
    },
    password: {
        ko: "비밀번호",
        en: "Password"
    },
    find_email: {
        ko: "이메일 찾기",
        en: "Find Email"
    },
    reset_pw: {
        ko: "비밀번호재설정",
        en: "Reset Password"
    },
    create_account: {
        ko: "회원가입",
        en: "Create Account"
    },
    check_title: {
        ko: "확인하세요!",
        en: "Check it out!"
    },
    characters: {
        ko: "- 비밀번호는 영문, 숫자, 특수기호 조합으로 8자 이상 구성되어야 합니다.",
        en: "- The password must consist of at least 8 characters and a combination of letters, numbers, and special symbols."
    },

    not_matched_error: {
        ko: "이메일 또는 비밀번호를 잘못 입력하셨습니다.",
        en: "You entered your email or password incorrectly."
    },
    not_exists_user_error: {
        ko: "존재하지 않는 유저입니다.",
        en: "This user does not exist."
    },
    login_failed_error: {
        ko: "로그인을 실패하셨습니다. 잠시 후 다시 시도해보세요.",
        en: "You have failed to log in. Please try again later."
    },
}