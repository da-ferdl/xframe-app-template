package one.lopes.eguiapp;

import com.google.androidgamesdk.GameActivity;

import android.annotation.SuppressLint;
import android.content.Context;
import android.os.Bundle;
import android.os.Build.VERSION;
import android.os.Build.VERSION_CODES;
import android.os.Handler;
import android.os.Looper;
import android.util.AttributeSet;
import android.util.Log;
import android.view.View;
import android.view.Window;
import android.view.WindowManager;

import androidx.activity.BackEventCompat;
import androidx.activity.EdgeToEdge;
import androidx.activity.OnBackPressedCallback;
import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.core.graphics.Insets;
import androidx.core.view.DisplayCutoutCompat;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowInsetsCompat;

public class MainActivity extends GameActivity {

    static {
        mainHandler = new Handler(Looper.getMainLooper());
        System.loadLibrary("eframe_my_app");
    }

    static Handler mainHandler;

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);

        if (hasFocus) {
            configureStatusBarForFullscreenExperience();
        }
    }

    /**
     * Configures the status bar for a fullscreen experience.
     *
     * <p>On API levels before 35, this sets a translucent status bar. On API level 35 and above, this
     * is a no-op as the system handles the status bar appearance, resulting in a fully transparent
     * status bar.
     */
    private void configureStatusBarForFullscreenExperience() {
        Window window = getWindow();
        if (VERSION.SDK_INT >= VERSION_CODES.Q) {
            //window.setSystemGestureExclusionRects();
        }
        window.addFlags(WindowManager.LayoutParams.FLAG_DRAWS_SYSTEM_BAR_BACKGROUNDS);
        if (VERSION.SDK_INT < VERSION_CODES.VANILLA_ICE_CREAM) {
            window.setStatusBarColor(0x40000000);
        }
        window.getDecorView().setSystemUiVisibility(View.SYSTEM_UI_FLAG_LAYOUT_STABLE | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN);
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        setTheme(R.style.AppTheme);
        // When true, the app will fit inside any system UI windows.
        // When false, we render behind any system UI windows.
        //WindowCompat.setDecorFitsSystemWindows(getWindow(), false);
        // You can set IME fields here or in native code using GameActivity_setImeEditorInfoFields.
        // We set the fields in native_engine.cpp.
        // super.setImeEditorInfoFields(InputType.TYPE_CLASS_TEXT,
        //     IME_ACTION_NONE, IME_FLAG_NO_FULLSCREEN );
        super.onCreate(savedInstanceState);

        configureStatusBarForFullscreenExperience();

        //####### Back Press Intercept #########
        OnBackPressedCallback callback = new OnBackPressedCallback(true ) {

            BackEventCompat lastBackEvent = null;

            @Override
            public void handleOnBackPressed() {
                // Handles the back button event
                handleTouchEnd();
            }

            @Override
            public void handleOnBackStarted(@NonNull BackEventCompat backEvent) {
                final float x_location =  backEvent.getTouchX();
                final float y_location = backEvent.getTouchX();

                lastBackEvent = backEvent;

                onBackPressLocation(x_location, y_location, (short) 1);

                super.handleOnBackStarted(backEvent);
            }

            @Override
            public void handleOnBackProgressed(@NonNull BackEventCompat backEvent) {
                final float x_location =  backEvent.getTouchX();
                final float y_location = backEvent.getTouchX();

                lastBackEvent = backEvent;

                onBackPressLocation(x_location, y_location, (short) 2);

                super.handleOnBackProgressed(backEvent);
            }

            @Override
            public void handleOnBackCancelled() {
                handleTouchEnd();

                super.handleOnBackCancelled();
            }

            private void handleTouchEnd() {
                float x_location = 0;
                float y_location = 0;

                if (lastBackEvent != null ){
                    x_location =  lastBackEvent.getTouchX();
                    y_location = lastBackEvent.getTouchX();
                }

                onBackPressLocation(x_location, y_location, (short) 3);
            }
        };
        getOnBackPressedDispatcher().addCallback(this, callback);


        //######## Safe Area ##########
        // Listener for display insets (cutouts) to pass values into native code.
        View content = getWindow().getDecorView().findViewById(android.R.id.content);
        ViewCompat.setOnApplyWindowInsetsListener(content, (view, insets) -> {
            onWindowInsetsEvent(view, insets);
            mainHandler.post(() -> {
                onWindowInsetsEvent(view, insets);
            });

            return insets;
        });
    }

    @Override
    public void onGlobalLayout() {
        super.onGlobalLayout();

        Log.d("-> App/", "onGlobalLayout: ");
    }

    /*@Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        if (requestCode == BleSession.COMPANION_CHOOSER_REQUEST_CODE) {
            if (resultCode == Activity.RESULT_OK && data != null) {
                BleSession.onCompanionChooserResult(resultCode, data);
            }
        } else {
            super.onActivityResult(requestCode, resultCode, data);
        }
    }*/

    /*public boolean isGooglePlayGames() {
        PackageManager pm = getPackageManager();
        return pm.hasSystemFeature("com.google.android.play.feature.HPE_EXPERIENCE");
    }*/

    protected void onWindowInsetsEvent(View view, WindowInsetsCompat insets) {

        // Setup cutouts values.
        DisplayCutoutCompat dc = insets.getDisplayCutout();
        int cutoutTop = 0;
        int cutoutRight = 0;
        int cutoutBottom = 0;
        int cutoutLeft = 0;
        if (dc != null) {
            cutoutTop = dc.getSafeInsetTop();
            cutoutRight = dc.getSafeInsetRight();
            cutoutBottom = dc.getSafeInsetBottom();
            cutoutLeft = dc.getSafeInsetLeft();
        }

        // Get display insets.
        Insets systemBars = insets.getInsets(WindowInsetsCompat.Type.systemBars());

        // Pass values into native code.
        onDisplayInsets(
                pxToDp(Integer.max(cutoutTop, systemBars.top)),
                pxToDp(Integer.max(cutoutRight, systemBars.right)),
                pxToDp(Integer.max(cutoutBottom, systemBars.bottom)),
                pxToDp(Integer.max(cutoutLeft, systemBars.left))
        );
    }

    protected int pxToDp(int px) {
        return (int) (px / this.getResources().getDisplayMetrics().density);
    }

    private static native void onDisplayInsets(int topInset, int rightInset, int bottomInset, int leftInset);

    private static native void onBackPressLocation(float x_location, float y_location, short touch_phase);
}