use std::cell::RefCell;
use std::ops::ControlFlow;
use windows::UI::Xaml::GridLengthHelper;
use windows::core::IInspectable;
use windows::core::implement;
use windows::Foundation::Collections::IVector;
use windows::UI::Xaml::Interop::TypeName;
use windows_app::Microsoft::UI::Xaml::ThicknessHelper;
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
use windows_app::Microsoft::UI::Xaml::Controls;
use windows_app::Microsoft::UI::Xaml::Controls::Primitives;

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

        let grid = Controls::Grid::new()?;
        let col1 = Controls::ColumnDefinition::new()?;
        col1.SetMaxWidth(200f64);
        let col2 = Controls::ColumnDefinition::new()?;
        grid.ColumnDefinitions()?.Append(col1);
        grid.ColumnDefinitions()?.Append(col2);

        let stackpanel = Controls::StackPanel::new()?;
        stackpanel.SetValue(Controls::Grid::ColumnProperty()?,IInspectable::try_from(0)?);

        let button = Controls::Button::new()?;
        button.SetContent(IInspectable::try_from("Button")?);
        button.SetMargin(ThicknessHelper::FromUniformLength(10f64)?);
        stackpanel.Children()?.Append(button);
        
        let dropdownbutton = Controls::DropDownButton::new()?;
        dropdownbutton.SetContent(IInspectable::try_from("DropDownButton")?);
        dropdownbutton.SetMargin(ThicknessHelper::FromUniformLength(10f64)?);
        stackpanel.Children()?.Append(dropdownbutton);

        let togglebutton = Controls::Primitives::ToggleButton::new()?;
        togglebutton.SetContent(IInspectable::try_from("ToggleButton")?);
        togglebutton.SetMargin(ThicknessHelper::FromUniformLength(10f64)?);
        stackpanel.Children()?.Append(togglebutton);
        
        let splitbutton = Controls::SplitButton::new()?;
        splitbutton.SetContent(IInspectable::try_from("SplitButton")?);
        splitbutton.SetMargin(ThicknessHelper::FromUniformLength(10f64)?);
        stackpanel.Children()?.Append(splitbutton);

        let togglesplitbutton = Controls::ToggleSplitButton::new()?;
        togglesplitbutton.SetContent(IInspectable::try_from("ToggleSplitButton")?);
        togglesplitbutton.SetMargin(ThicknessHelper::FromUniformLength(10f64)?);
        stackpanel.Children()?.Append(togglesplitbutton);

        let combobox = Controls::ComboBox::new()?;
        combobox.Items()?.Append(IInspectable::try_from("ComboBox")?);
        combobox.SetMargin(ThicknessHelper::FromUniformLength(10f64)?);
        stackpanel.Children()?.Append(combobox);

        let radiobutton = Controls::RadioButton::new()?;
        radiobutton.SetContent(IInspectable::try_from("RadioButton")?);
        radiobutton.SetMargin(ThicknessHelper::FromUniformLength(10f64)?);
        stackpanel.Children()?.Append(radiobutton);

        let ratingcontrol = Controls::RatingControl::new()?;
        ratingcontrol.SetMargin(ThicknessHelper::FromUniformLength(10f64)?);
        stackpanel.Children()?.Append(ratingcontrol);

        let slider = Controls::Slider::new()?;
        slider.SetMargin(ThicknessHelper::FromUniformLength(10f64)?);
        stackpanel.Children()?.Append(slider);

        let toggleswitch = Controls::ToggleSwitch::new()?;
        toggleswitch.SetMargin(ThicknessHelper::FromUniformLength(10f64)?);
        stackpanel.Children()?.Append(toggleswitch);

        let colorpicker = Controls::ColorPicker::new()?;
        colorpicker.SetValue(Controls::Grid::ColumnProperty()?,IInspectable::try_from(1)?);
        //Controls::Grid::SetColumn(colorpicker, 1);

        grid.Children()?.Append(stackpanel);
        grid.Children()?.Append(colorpicker);
        window.SetContent(grid);

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
