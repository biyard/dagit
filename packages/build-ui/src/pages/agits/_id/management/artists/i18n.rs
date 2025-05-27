use bdk::prelude::*;

translate! {
    ArtistTranslate;
    artist:{
        en: "Artist",
        ko: "작가",
    }
    artists:{
        en: "Artists",
        ko: "작가 목록",
    }
    total_artists:{
        en: "Total Artists",
        ko: "전체 작가",
    }
    new_artist:{
        en: "New Artist",
        ko: "신규 작가",
    }
    remove_artist:{
        en: "Remove Artist",
        ko: "작가 제거",
    }
    status:{
        en: "Status",
        ko: "상태",
    }
    featured_work:{
        en: "Featured Work",
        ko: "대표작",
    }
    attributes:{
        en: "Attributes",
        ko: "속성",
    }
    artworks:{
        en: "Artworks",
        ko: "작품",
    }
    revenue:{
        en: "Revenue",
        ko: "수익",
    }
    social_media:{
        en: "Social Media",
        ko: "소셜 미디어",
    }
    search_by_title:{
        en: "Search by Title",
        ko: "제목으로 검색",
    }
    all:{
        en: "All",
        ko: "전체",
    }
    medium:{
        en: "Medium",
        ko: "매체",
    }
    rarity:{
        en: "Rarity",
        ko: "희귀성",
    }
    owner:{
        en: "Owner",
        ko: "소유자",
    }
    current_price:{
        en: "Current Price",
        ko: "현재 가격",
    }
    average_price:{
        en: "Average Price",
        ko: "평균 가격",
    }
    price_change:{
        en: "Price Change",
        ko: "가격 변화",
    }
    sales_volume:{
        en: "Sales Volume",
        ko: "판매량",
    }
    title:{
        en: "Title",
        ko: "제목",
    }
}

translate! {
    ConfirmRemoveArtistModalTranslate;

    title:{
        en: "Artist Removal Confirmation",
        ko: "작가 제거 확인",
    }
    popup_description:{
        en: "Removing an artist is irreversible. Once removed, all artworks associated with the artist will be converted to `Pending Sale` status and will no longer be available for sale from that point forward.",
        ko: "작가를 제거하는 것은 되돌릴 수 없습니다. 제거되면 해당 작가와 관련된 모든 작품은 '판매 보류' 상태로 전환되며 그 시점부터 판매할 수 없습니다.",
    }
    popup_description_2:{
        en: "Are you sure you want to remove the artist?",
        ko: "위 정보를 읽고 이해했음을 확인해 주십시오.",
    }
    popup_confirm_text:{
        en:"I acknowledge and accept the permanent removal of the artist and the legal consequences of this action, including the loss of associated rights, royalties, and future sales opportunities. I understand that this action is final and cannot be undone.",
        ko:"작가의 영구 제거와 이 조치의 법적 결과, 즉 관련 권리, 로열티 및 향후 판매 기회의 상실을 인정하고 수용합니다. 이 조치는 최종적이며 되돌릴 수 없음을 이해합니다.",
    }
    confirm_btn:{
        en: "Confirm",
        ko: "확인",
    }
    cancel_btn:{
        en: "Cancel",
        ko: "취소",
    }


}

translate! {
    RemovalSuccessModalTranslate;
        title:{
        en: "Success",
        ko: "성공",
    }
    success_popup_description:{
        en: "The artist has been removed from your agit. Returning to the previous screen.",
        ko: "작가가 아짓에서 제거되었습니다. 이전 화면으로 돌아갑니다.",
    }
    confirm_btn:{
        en: "Confirm",
        ko: "확인",
    }
}

translate! {
    RemoveArtistNameModalTranslate;
    confirm_btn:{
        en: "Confirm",
        ko: "확인",
    }
    cancel_btn:{
        en: "Cancel",
        ko: "취소",
    }
    title:{
        en:"Remove the Artist from your agit",
        ko: "작가를 아짓에서 제거",

    }
    sub_title:{
        en: "Please note that removed artists cannot be restored."
        ko: "제거된 작가는 복원할 수 없습니다. 삭제하려는 작가의 이름을 입력하세요.",
    }
    enter_artist_name:{
        en: "Enter the name of the artist you want to delete.",
        ko: "삭제하려는 작가의 이름을 입력하세요.",
    }
    popup_confirm_text:{
        en:"I have read and accept the Terms of Service.",
        ko:"서비스 약관을 읽고 동의합니다.",
    }

}
