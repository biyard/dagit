use bdk::prelude::*;

translate! {
    CollectionTranslate;
    collection: {
        en: "Collection",
        ko: "컬렉션",
    },

    collections: {
        en: "Collections",
        ko: "컬렉션 목록",
    },

    floor_price: {
        en: "Floor price",
        ko: "최저가",
    },

    floor_change: {
        en: "Floor Change",
        ko: "최저가 변화",
    },

    volume_change: {
        en: "Volume Change",
        ko: "거래량 변화",
    },

    owners: {
        en: "Owners",
        ko: "소유자",
    },

    stock: {
        en: "Stock",
        ko: "보유 수량",
    },
     new_collection: {
        en: "New Collection",
        ko: "신규 컬렉션",
    },
     add_artwork: {
        en: "Add Artwork",
        ko: "작품 추가",
    },
     remove_artwork: {
        en: "Remove Artwork",
        ko: "작품 제거",
    },
    total_collection: {
        en: "Total Collections",
        ko: "전체 컬렉션",
    },

    total_volume: {
        en: "Total Volume",
        ko: "총 거래량",
    },
    title:{
        en: "Title",
        ko: "제목",
    }
    owned: {
        en: "Owned",
        ko: "소유한",
    },
    owner: {
        en: "Owner",
        ko: "소유자",
    },

    token_id: {
        en: "Token Id",
        ko: "토큰 ID",
    },

    wallet_address: {
        en: "Wallet Address",
        ko: "지갑 주소",
    },

    last_activity: {
        en: "Last Activity",
        ko: "마지막 활동",
    },
    attributes:{
        en: "Attributes",
        ko: "속성",
    }
    ways_to_sell:{
        en: "Ways to Sell",
        ko: "판매 방법",
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
    volume:{
        en: "Volume",
        ko: "거래량",
    }
    sales_volume:{
        en: "Sales Volume",
        ko: "판매량",
    }
    royalty:{
        en: "Royalty",
        ko: "로열티",
    }
    status:{
        en: "Status",
        ko: "상태",
    }
    trade:{
        en: "Trade",
        ko: "거래",
    },
    created:{
        en: "Created",
        ko: "생성됨",
    },
   search_by_title:{
        en: "Search by title",
        ko: "제목으로 검색",
    }
      list:{
        en:"List",
        ko:"리스트",
    }
    activity:{
        en:"Activity",
        ko:"활동",
    }


}

translate! {
    NewCollectionModalTranslate;
    title:{
        en: "Please select the artwork to include in the collection",
        ko: "컬렉션에 포함할 작품을 선택하세요",
    }
    sub_title:{
        en: "Add artwork to the collection. You can also move artwork from another collection.",
        ko: "컬렉션에 작품을 추가하세요. 다른 컬렉션에서 작품을 이동할 수도 있습니다.",
    }
    search_placeholder:{
        en: "Search by title",
        ko: "제목으로 검색",
    }
    cancel_btn_txt:{
        en: "Cancel",
        ko: "취소",
    }
    confirm_btn_txt:{
        en: "Confirm",
        ko: "확인",
    }
    artwork_selected:{
        en:"artworks have been selected.",
        ko:"작품이 선택되었습니다.",
    }


}

translate! {
    TransferConfirmationModalTranslate;

    title:{
        en: "Transfer Artwork",
        ko: "작품 전송",
    }
    description:{
        en: "artworks are already included in another collection. Would you like to transfer the artworks to the designated collection?",
        ko: "개의 선택된 작품은 이미 다른 컬렉션에 포함되어 있습니다. 지정된 컬렉션으로 작품을 전송하시겠습니까?",
    }
    back_btn_text:{
        en: "Back",
        ko: "백",

    }
    continue_btn_text:{
        en: "Continue",
        ko: "계속",
    }

}

translate! {
    CollectionNameInputModalTranslate;
    title:{
        en: "What is the name of collection?",
        ko: "컬렉션의 이름은 무엇인가요?",
    }
    sub_title:{
        en:"Choose a collection name.",
        ko:"컬렉션 이름을 선택하세요.",
    }
    collection_name_placeholder:{
        en: "Collection Name",
        ko: "컬렉션 이름",
    }
    short_url_placeholder:{
        en: "Short URL",
        ko: "짧은 URL",
    }
    collection_name:{
        en: "Collection Name",
        ko: "컬렉션 이름",
    }
    short_url:{
        en: "Short URL",
        ko: "짧은 URL",
    }
    collection_name_error:{
        en: "Collection name is required",
        ko: "컬렉션 이름은 필수입니다",
    }
    short_url_error:{
        en: "Short URL is required",
        ko: "짧은 URL은 필수입니다",
    }
    collection_name_invalid:{
        en: "Collection name is invalid",
        ko: "컬렉션 이름이 유효하지 않습니다",
    }
    short_url_invalid:{
        en: "Short URL is invalid",
        ko: "짧은 URL이 유효하지 않습니다",
    }
    collection_name_taken:{
        en: "Collection name is already taken",
        ko: "컬렉션 이름이 이미 사용 중입니다",
    }
    short_url_taken:{
        en: "Short URL is already taken",
        ko: "짧은 URL이 이미 사용 중입니다",
    }
    collection_name_success:{
        en: "Collection name is available",
        ko: "컬렉션 이름이 사용 가능합니다",
    }
    add_btn_text:{
        en: "Add Collection",
        ko: "추가",
    }
    back_btn_text:{
        en: "Back",
        ko: "백",
    }


}

translate! {
    SuccessModalTranslate;
    title:{
        en: "Success",
        ko: "성공",
    }
    description:{
        en: "The collection has been successfully created. Redirecting to the collection screen.",
        ko: "컬렉션이 성공적으로 생성되었습니다. 컬렉션 화면으로 리디렉션됩니다.",
    }
    confirm_btn_text:{
        en: "Confirm",
        ko: "확인",
    }
}
