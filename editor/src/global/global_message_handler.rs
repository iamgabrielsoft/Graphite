use crate::message_prelude::*;

#[impl_message(Message, Global)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum GlobalMessage {
	LogInfo,
	LogDebug,
	LogTrace,
	RecordInput(Box<Message>),
	RecordOutput(FrontendMessage),
	ExportTrace,
}

#[derive(Debug, Default)]
pub struct GlobalMessageHandler {
	input_messages: Vec<Message>,
	output_messages: Vec<FrontendMessage>,
}

impl MessageHandler<GlobalMessage, ()> for GlobalMessageHandler {

  #[remain::check]
	fn process_action(&mut self, message: GlobalMessage, _data: (), responses: &mut VecDeque<Message>) {
		use GlobalMessage::*;

		#[remain::sorted]
		match message {
			LogDebug => {
				log::set_max_level(log::LevelFilter::Debug);
				log::info!("Set log verbosity to debug");
			}
			LogInfo => {
				log::set_max_level(log::LevelFilter::Info);
				log::info!("Set log verbosity to info");
			}
			LogTrace => {
				log::set_max_level(log::LevelFilter::Trace);
				log::info!("Set log verbosity to trace");
			}
			RecordInput(message) => self.input_messages.push(*message),
			RecordOutput(message) => self.output_messages.push(message),
			ExportTrace => {
				let mut case = TestCase::default();
				std::mem::swap(&mut self.input_messages, &mut case.input_messages);
				std::mem::swap(&mut self.output_messages, &mut case.output_messages);
				responses.push_back(
					FrontendMessage::ExportDocument {
						document: ron::ser::to_string_pretty(&case, ron::ser::PrettyConfig::default()).expect("Failed to serialize message trace"),
						name: String::from("test_case.ron"),
					}
					.into(),
				);
			}
		}
	}
	advertise_actions!(GlobalMessageDiscriminant; LogInfo, LogDebug, LogTrace, ExportTrace);
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Default)]
struct TestCase {
	input_messages: Vec<Message>,
	output_messages: Vec<FrontendMessage>,
}
