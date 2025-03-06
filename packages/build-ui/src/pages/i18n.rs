#![allow(unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::*;

translate! {
    RootTranslate;

    solution: {
        en: "Solution",
        ko: "솔루션",
    },

    pricing: {
        en: "Pricing",
        ko: "가격",
    },

    faq: {
        en: "FAQ",
        ko: "자주 묻는 질문",
    },

    login: {
        en: "Login",
        ko: "로그인",
    },

    description: {
        en: "Blockchain-based artwork certificates, seamless data management, digital gallery solutions.",
        ko: "블록체인 기반의 예술 작품 인증서, 원활한 데이터 관리, 디지털 갤러리 솔루션.",
    },

    unlock_possibilities: {
        en: "Let's unlock new possibilities with your own Agit today!",
        ko: "지금 바로 당신만의 아지트를 만들어 새로운 가능성을 열어보세요!",
    },

    build_agit: {
        en: "Build your Agit",
        ko: "아지트 만들기",
    }
}
