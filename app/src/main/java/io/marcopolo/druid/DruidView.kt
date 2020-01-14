package io.marcopolo.druid

import android.content.Context
import android.graphics.Canvas
import android.os.Handler
import android.os.Looper
import android.util.Log
import android.view.MotionEvent
import android.view.View

class DruidView(context: Context): View(context) {
    var uiHandler = Handler(Looper.getMainLooper())

    var druidMessageCallback = Handler.Callback {
        // Idle callback
        if (it.arg1 == 0) {
            uiHandler.post {
                this.onIdle()
            }
        // Timer
        } else {
            val timerId = it.arg1
            val timerDuration = it.arg2
            uiHandler.postDelayed({
                this.onTimer(timerId)
            }, timerDuration.toLong())

        }

        true
    }

    var druidHandler = Handler(Looper.getMainLooper(), druidMessageCallback)


    external fun setup(context: Context, druid_view: View, android_handler: Handler)

    init {
        val density = context.getResources().getDisplayMetrics().density
        Log.d("DRUIDVIEW", "Density is $density")
        setup(context, this, druidHandler)
    }


    external fun onIdle()
    external fun onTimer(token_id: Int)

    external override fun onDraw(canvas: Canvas)
    external override fun onSizeChanged (w: Int, h: Int, oldw: Int, oldh: Int)

    external override fun onTouchEvent(e: MotionEvent): Boolean
}
