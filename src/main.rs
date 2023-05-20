use std::cell::RefCell;
use windows::core::IInspectable;
use windows::core::implement;
use windows::Foundation::Collections::IVector;
use windows::UI::Xaml::Interop::TypeName;
use windows_app::bootstrap;
use windows_app::Microsoft::UI::Xaml::Application;
use windows_app::Microsoft::UI::Xaml::IApplicationOverrides;
use windows_app::Microsoft::UI::Xaml::IApplicationOverrides_Impl;
use windows_app::Microsoft::UI::Xaml::ApplicationInitializationCallback;
use windows_app::Microsoft::UI::Xaml::Markup::IXamlMetadataProvider_Impl;
use windows_app::Microsoft::UI::Xaml::Window;
use windows_app::Microsoft::UI::Xaml::ResourceDictionary;
use windows_app::Microsoft::UI::Xaml::LaunchActivatedEventArgs;
use windows_app::Microsoft::UI::Xaml::Controls::XamlControlsResources;
use windows_app::Microsoft::UI::Xaml::Controls::ColorPicker;
use windows_app::Microsoft::UI::Xaml::Controls::RatingControl;
use windows_app::Microsoft::UI::Xaml::Controls::ToggleSplitButton;
use windows_app::Microsoft::UI::Xaml::Markup::XmlnsDefinition;
use windows_app::Microsoft::UI::Xaml::Markup::IXamlMetadataProvider;
use windows_app::Microsoft::UI::Xaml::XamlTypeInfo::XamlControlsXamlMetaDataProvider;

#[implement(IApplicationOverrides,IXamlMetadataProvider)]
struct App {
    window: RefCell<Option<Window>>,
    provider: XamlControlsXamlMetaDataProvider
}

#[allow(non_snake_case)]
impl App {
    fn new() -> windows::core::Result<App> {
        let app = App {
            window: RefCell::new(None),
            provider: XamlControlsXamlMetaDataProvider::new()?
        };
        Ok(app)
    }
}

impl IApplicationOverrides_Impl for App {
    fn OnLaunched(&self, _: &Option<LaunchActivatedEventArgs>) -> windows::core::Result<()> {
        Application::Current()?.Resources()?.MergedDictionaries()?.Append(XamlControlsResources::new()?);
        let window = Window::new()?;
        window.SetContent(ColorPicker::new()?);

        let result = window.Activate();
        *self.window.borrow_mut() = Some(window);
        result
    }
}

impl IXamlMetadataProvider_Impl for App{
    fn GetXamlType(&self,r#type: &TypeName,) ->  windows::core::Result<windows_app::Microsoft::UI::Xaml::Markup::IXamlType> {
        self.provider.GetXamlType(r#type)
    }
    
    fn GetXamlTypeByFullName(&self,fullname: &windows::core::HSTRING,) ->  windows::core::Result<windows_app::Microsoft::UI::Xaml::Markup::IXamlType> {
        self.provider.GetXamlTypeByFullName(fullname)
    }
    
    fn GetXmlnsDefinitions(&self,) ->  windows::core::Result<windows::core::Array<XmlnsDefinition>> {
        self.provider.GetXmlnsDefinitions()
    }
}

fn main() -> windows::core::Result<()> {
    bootstrap::initialize()?;

    Application::Start(ApplicationInitializationCallback::new(|_| {
        let _ = Application::compose(App::new()?)?;
        Ok(())
    }))?;

    bootstrap::uninitialize()
}
