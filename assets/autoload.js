/*!
 * Live2D Widget
 * https://github.com/stevenjoezhang/live2d-widget
 */

// Resolve assets from Trunk's public URL so local builds and GitHub Pages
// deployments use the same paths.
const public_base = document.querySelector('base')?.href || document.baseURI;
const live2d_path = new URL('widget/', public_base).href;
const live2d_model_path = new URL(
  '_assets/live2d/sen-ya-lulu/model3.json',
  public_base,
).href;
const live2d_tips_path = new URL(
  '_assets/live2d/waifu-tips.json',
  public_base,
).href;

// Method to encapsulate asynchronous resource loading
// 封装异步加载资源的方法
function loadExternalResource(url, type) {
  return new Promise((resolve, reject) => {
    let tag;

    if (type === 'css') {
      tag = document.createElement('link');
      tag.rel = 'stylesheet';
      tag.href = url;
    }
    else if (type === 'js') {
      tag = document.createElement('script');
      tag.type = 'module';
      tag.src = url;
    }
    if (tag) {
      tag.onload = () => resolve(url);
      tag.onerror = () => reject(url);
      document.head.appendChild(tag);
    }
  });
}

/**
 * Apply the model-specific parameter overrides declared in the adapted
 * model3.json. The widget's Cubism 5 demo does not expose an init-parameter
 * hook, so this small adapter uses its existing public model lifecycle and
 * persists the override in the model's parameter snapshot.
 */
async function installModelAdapter(model_path) {
  const response = await fetch(model_path);
  if (!response.ok) {
    throw new Error(`Unable to load Live2D model settings: ${model_path}`);
  }
  const settings = await response.json();
  const overrides = settings.BlogAdaptation?.parameterOverrides || [];
  if (!overrides.length) return;

  const { AppDelegate } = await import(new URL('chunk/index2.js', live2d_path).href);
  const originalChangeModel = AppDelegate.prototype.changeModel;
  if (originalChangeModel.blogModelAdapterInstalled) return;

  const changeModel = function(model_setting_path) {
    originalChangeModel.call(this, model_setting_path);

    let attempts = 0;
    const applyOverrides = () => {
      const subdelegate = this._subdelegates?.at(0);
      const live2d_manager = subdelegate?.getLive2DManager?.();
      const model = live2d_manager?._models?.at(0);
      const live2d_model = model?._model;

      if (live2d_model?.getParameterCount && live2d_model?.setParameterValueByIndex) {
        for (let index = 0; index < live2d_model.getParameterCount(); index += 1) {
          const id = live2d_model.getParameterId(index);
          const raw_name = id?.getString?.();
          const name = typeof raw_name === 'string' ? raw_name : raw_name?.s;
          const override = overrides.find(item => item.Id === name);
          if (!override) continue;

          live2d_model.setParameterValueByIndex(index, override.Value);
          live2d_model.saveParameters?.();
          return;
        }
      }

      if (attempts < 180) {
        attempts += 1;
        window.requestAnimationFrame(applyOverrides);
      }
    };

    window.requestAnimationFrame(applyOverrides);
  };
  changeModel.blogModelAdapterInstalled = true;
  AppDelegate.prototype.changeModel = changeModel;
}

(async () => {
  // If you are concerned about display issues on mobile devices, you can use screen.width to determine whether to load
  // 如果担心手机上显示效果不佳，可以根据屏幕宽度来判断是否加载
  if (screen.width < 768) return;

  // Avoid cross-origin issues with image resources
  // 避免图片资源跨域问题
  const OriginalImage = window.Image;
  window.Image = function(...args) {
    const img = new OriginalImage(...args);
    img.crossOrigin = "anonymous";
    return img;
  };
  window.Image.prototype = OriginalImage.prototype;
  // Load waifu.css and waifu-tips.js
  // 加载 waifu.css 和 waifu-tips.js
  await Promise.all([
    loadExternalResource(live2d_path + 'waifu.css', 'css'),
    loadExternalResource(live2d_path + 'waifu-tips.js', 'js')
  ]);
  await installModelAdapter(live2d_model_path);
  // For detailed usage of configuration options, see README.en.md
  // 配置选项的具体用法见 README.md
  initWidget({
    waifuPath: live2d_tips_path,
    // cdnPath: 'https://fastly.jsdelivr.net/gh/fghrsh/live2d_api/',
    cubism2Path: live2d_path + 'live2d.min.js',
    cubism5Path: 'https://cubism.live2d.com/sdk-web/cubismcore/live2dcubismcore.min.js',
    tools: ['hitokoto', 'asteroids', 'switch-model', 'switch-texture', 'photo', 'info', 'quit'],
    logLevel: 'warn',
    drag: false,
  });
})();

console.log(`\n%cLive2D%cWidget%c\n`, 'padding: 8px; background: #cd3e45; font-weight: bold; font-size: large; color: white;', 'padding: 8px; background: #ff5450; font-size: large; color: #eee;', '');

/*
く__,.ヘヽ.        /  ,ー､ 〉
         ＼ ', !-─‐-i  /  /´
         ／｀ｰ'       L/／｀ヽ､
       /   ／,   /|   ,   ,       ',
     ｲ   / /-‐/  ｉ  L_ ﾊ ヽ!   i
      ﾚ ﾍ 7ｲ｀ﾄ   ﾚ'ｧ-ﾄ､!ハ|   |
        !,/7 '0'     ´0iソ|    |
        |.从"    _     ,,,, / |./    |
        ﾚ'| i＞.､,,__  _,.イ /   .i   |
          ﾚ'| | / k_７_/ﾚ'ヽ,  ﾊ.  |
            | |/i 〈|/   i  ,.ﾍ |  i  |
           .|/ /  ｉ：    ﾍ!    ＼  |
            kヽ>､ﾊ    _,.ﾍ､    /､!
            !'〈//｀Ｔ´', ＼ ｀'7'ｰr'
            ﾚ'ヽL__|___i,___,ンﾚ|ノ
                ﾄ-,/  |___./
                'ｰ'    !_,.:
*/
