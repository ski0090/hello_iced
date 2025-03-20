리스트 뷰에 대한 위젯을 찾고 있습니다
원하는 형태의 위젯은 보지 못하였습니다

- 컬럼에 대한 정보를 표시해 줌
- 컬럼을 기준으로 정렬가능
- 리스트 Item에 대한 클릭 이벤트를 호출
- 콤보박스 형식이 아니어야 함

따라서 [커스텀 위젯](https://github.com/iced-rs/awesome-iced?tab=readme-ov-file#custom-widgets)에서
추가적인 위젯을 구현한 [iced-aw](https://github.com/iced-rs/iced_aw) 분석하였습니다

해당 crate엔 다음과 같은 위젯들이 구현되어 있습니다

- Badge: 색 배경을 가진 text
- Card: 메세지 박스
- ColorPicker: 색상 선택 위젯
- DatePicker: 달력 위젯
- NumberInput: 숫자만 입력할 수 있는 인풋
- SelectionList: 콤보박스, 현재는 iced에 pick_list와 같은 기능을 하는 것으로 보임
- TabBar and Tabs: gtk 기준으로 Stack과 StackSwitch
- TimePicker: 시계 위젯
- Menu: 메뉴 바
- SideBar: 네비게이션 바
