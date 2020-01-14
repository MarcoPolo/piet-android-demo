package io.marcopolo.pietdemo

import android.content.Context
import android.inputmethodservice.KeyboardView
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.system.Os
import android.util.Log
import android.view.KeyEvent
import android.view.View
import android.view.ViewGroup
import android.view.inputmethod.InputMethodManager
import android.widget.EditText
import io.marcopolo.druid.DruidView

class MainActivity : AppCompatActivity(), KeyEvent.Callback {

    override fun onKeyUp(keyCode: Int, event: KeyEvent?): Boolean {
        Log.d("MainActivity", "Key code: $keyCode – $event")
        return super.onKeyUp(keyCode, event)
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        System.loadLibrary("pietdemo")
        initRust(cacheDir.absolutePath)
//        setContentView(PietDemoView(this))
        setContentView(DruidView(this))
        startDruid()

    }

    external fun initRust(cachePath: String)
    external fun startDruid()
}
