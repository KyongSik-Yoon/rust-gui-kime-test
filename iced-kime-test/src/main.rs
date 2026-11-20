use iced::widget::{button, column, container, operation, row, scrollable, text, text_editor, text_input};
use iced::{Element, Event, Fill, Font, Subscription, Task, Theme};

const SINGLE_INPUT_ID: &str = "single_input";

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("iced 0.14 + kime 한글 입력 검증")
        .default_font(Font::with_name("Noto Sans CJK KR"))
        .theme(|_: &App| Theme::TokyoNight)
        .subscription(App::subscription)
        .run()
}

struct App {
    single_line: String,
    multi_line: text_editor::Content,
    messages: Vec<String>,
    ime_logs: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    SingleChanged(String),
    SingleSubmitted,
    EditorAction(text_editor::Action),
    EditorSubmitted,
    ClearLogs,
    RawEvent(Event),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                single_line: String::new(),
                multi_line: text_editor::Content::new(),
                messages: vec!["여기에 전송된 메시지가 표시됩니다. (한글 타자·백스페이스·Enter·포커스 전환 테스트)".into()],
                ime_logs: Vec::new(),
            },
            operation::focus(SINGLE_INPUT_ID),
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::RawEvent)
    }

    fn push_log(&mut self, line: String) {
        self.ime_logs.push(line);
        if self.ime_logs.len() > 10 {
            let drain = self.ime_logs.len() - 10;
            self.ime_logs.drain(0..drain);
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SingleChanged(val) => {
                self.single_line = val;
            }
            Message::SingleSubmitted => {
                let s = std::mem::take(&mut self.single_line);
                if !s.is_empty() {
                    self.messages.push(format!("[text_input] {s}"));
                }
            }
            Message::EditorAction(action) => {
                self.multi_line.perform(action);
            }
            Message::EditorSubmitted => {
                let s = self.multi_line.text();
                let trimmed = s.trim_end_matches('\n').to_string();
                if !trimmed.is_empty() {
                    self.messages.push(format!("[text_editor] {trimmed}"));
                }
                self.multi_line = text_editor::Content::new();
            }
            Message::ClearLogs => {
                self.ime_logs.clear();
                self.messages.clear();
            }
            Message::RawEvent(Event::InputMethod(ime)) => {
                self.push_log(format!("IME: {ime:?}"));
            }
            Message::RawEvent(Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key,
                physical_key,
                text,
                modifiers,
                ..
            })) => {
                self.push_log(format!(
                    "KEY: key={key:?} phys={physical_key:?} text={text:?} mod={modifiers:?}"
                ));
            }
            Message::RawEvent(_) => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let header = row![
            text("iced 0.14 + kime 한글 입력 검증").size(20),
            button("로그 초기화").on_press(Message::ClearLogs),
        ]
        .spacing(12);

        let single = column![
            text("1) 한 줄 입력 (text_input) — Enter로 전송").size(14),
            text_input("한글을 입력하고 Enter를 눌러보세요...", &self.single_line)
                .id(SINGLE_INPUT_ID)
                .on_input(Message::SingleChanged)
                .on_submit(Message::SingleSubmitted)
                .padding(10),
        ]
        .spacing(6);

        let multi = column![
            row![
                text("2) 여러 줄 입력 (text_editor) — 줄바꿈·백스페이스·드래그 테스트").size(14),
                button("전송").on_press(Message::EditorSubmitted),
            ]
            .spacing(10),
            text_editor(&self.multi_line)
                .placeholder("여러 줄 한글 입력, 조합 중 백스페이스/방향키/클릭 테스트...")
                .on_action(Message::EditorAction)
                .height(140)
                .padding(10),
        ]
        .spacing(6);

        let sent_list = scrollable(
            column(
                self.messages
                    .iter()
                    .map(|m| text(m).size(14).into())
                    .collect::<Vec<Element<'_, Message>>>(),
            )
            .spacing(4),
        )
        .height(140);

        let ime_box = scrollable(
            column(
                self.ime_logs
                    .iter()
                    .map(|l| text(l).size(12).into())
                    .collect::<Vec<Element<'_, Message>>>(),
            )
            .spacing(2),
        )
        .height(140);

        let content = column![
            header,
            single,
            multi,
            text("전송된 메시지:").size(14),
            container(sent_list).padding(8).width(Fill),
            text("실시간 IME 이벤트 로그 (Opened / Preedit / Commit / Closed):").size(14),
            container(ime_box).padding(8).width(Fill),
        ]
        .spacing(12)
        .padding(18);

        container(content).width(Fill).height(Fill).into()
    }
}
