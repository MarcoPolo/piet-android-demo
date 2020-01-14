package io.marcopolo.pietdemo

import android.content.Context
import android.content.Context.INPUT_METHOD_SERVICE
import android.graphics.Canvas
import android.util.Log
import android.view.KeyEvent
import android.view.MotionEvent
import android.view.View
import android.view.inputmethod.InputMethodManager
import android.widget.EditText


class PietDemoView(context: Context) : EditText(context), KeyEvent.Callback {
    private var kbShown: Boolean = false

    override fun onKeyPreIme(keyCode: Int, event: KeyEvent?): Boolean {
        Log.d("PietView", "PreIME - Key code: $keyCode – $event")
        return super.onKeyPreIme(keyCode, event)
    }

    override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
        Log.d("PietView", "size changed: $w $h $oldw $oldh")
        super.onSizeChanged(w, h, oldw, oldh)
    }

    override fun onTouchEvent(e: MotionEvent): Boolean {
        var isDown = e.action == MotionEvent.ACTION_DOWN
        Log.d("PietView", "Event: $e – $isDown")
        return true
    }
}
