package io.marcopolo.pietdemo

import android.content.Context
import android.graphics.*
import android.view.View

class PietDemoView(context: Context) : View(context) {
    external override fun onDraw(canvas: Canvas)
}
